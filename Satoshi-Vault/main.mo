import AccessControl "authorization/access-control";
import Principal "mo:base/Principal";
import OrderedMap "mo:base/OrderedMap";
import Time "mo:base/Time";
import Debug "mo:base/Debug";
import Text "mo:base/Text";
import List "mo:base/List";
import Iter "mo:base/Iter";

actor SatoshiVault {
  // Initialize the user system state
  let accessControlState = AccessControl.initState();

  // Initialize auth (first caller becomes admin, others become users)
  public shared ({ caller }) func initializeAccessControl() : async () {
    AccessControl.initialize(accessControlState, caller);
  };

  public query ({ caller }) func getCallerUserRole() : async AccessControl.UserRole {
    AccessControl.getUserRole(accessControlState, caller);
  };

  public shared ({ caller }) func assignCallerUserRole(user : Principal, role : AccessControl.UserRole) : async () {
    // Admin-only check happens inside
    AccessControl.assignRole(accessControlState, caller, user, role);
  };

  public query ({ caller }) func isCallerAdmin() : async Bool {
    AccessControl.isAdmin(accessControlState, caller);
  };

  public type UserProfile = {
    name : Text;
    // Other user metadata if needed
  };

  transient let principalMap = OrderedMap.Make<Principal>(Principal.compare);
  var userProfiles = principalMap.empty<UserProfile>();

  public query ({ caller }) func getCallerUserProfile() : async ?UserProfile {
    if (not (AccessControl.hasPermission(accessControlState, caller, #user))) {
      Debug.trap("Unauthorized: Only users can save profiles");
    };
    principalMap.get(userProfiles, caller);
  };

  public query ({ caller }) func getUserProfile(user : Principal) : async ?UserProfile {
    if (caller != user and not AccessControl.isAdmin(accessControlState, caller)) {
      Debug.trap("Unauthorized: Can only view your own profile");
    };
    principalMap.get(userProfiles, user);
  };

  public shared ({ caller }) func saveCallerUserProfile(profile : UserProfile) : async () {
    if (not (AccessControl.hasPermission(accessControlState, caller, #user))) {
      Debug.trap("Unauthorized: Only users can save profiles");
    };
    userProfiles := principalMap.put(userProfiles, caller, profile);
  };

  // Vault Types
  public type Vault = {
    primaryAddress : Text;
    backupAddress : Text;
    inactivityPeriod : Nat; // Days
    lastActive : Time.Time;
    created : Time.Time;
    ckBTCBalance : Nat;
  };

  public type EncryptedMessage = {
    encryptedData : Text;
    recipientAddress : Text;
    created : Time.Time;
  };

  public type ActivityLog = {
    timestamp : Time.Time;
    action : Text;
    details : Text;
  };

  // Storage
  var vaults = principalMap.empty<Vault>();
  var messages = principalMap.empty<List.List<EncryptedMessage>>();
  var activityLogs = principalMap.empty<List.List<ActivityLog>>();

  // Vault Management
  public shared ({ caller }) func createVault(primaryAddress : Text, backupAddress : Text, inactivityPeriod : Nat) : async () {
    if (not (AccessControl.hasPermission(accessControlState, caller, #user))) {
      Debug.trap("Unauthorized: Only users can create vaults");
    };

    let now = Time.now();
    let vault : Vault = {
      primaryAddress;
      backupAddress;
      inactivityPeriod;
      lastActive = now;
      created = now;
      ckBTCBalance = 0;
    };

    vaults := principalMap.put(vaults, caller, vault);
    addActivityLog(caller, "Vault Created", "Primary: " # primaryAddress # ", Backup: " # backupAddress);
  };

  public shared ({ caller }) func updateActivity() : async () {
    if (not (AccessControl.hasPermission(accessControlState, caller, #user))) {
      Debug.trap("Unauthorized: Only users can update activity");
    };

    switch (principalMap.get(vaults, caller)) {
      case (null) { Debug.trap("Vault not found") };
      case (?vault) {
        let updatedVault = {
          vault with
          lastActive = Time.now();
        };
        vaults := principalMap.put(vaults, caller, updatedVault);
        addActivityLog(caller, "Activity Updated", "Last active timestamp updated");
      };
    };
  };

  public query ({ caller }) func getVault() : async ?Vault {
    if (not (AccessControl.hasPermission(accessControlState, caller, #user))) {
      Debug.trap("Unauthorized: Only users can view vaults");
    };
    principalMap.get(vaults, caller);
  };

  // Encrypted Messages
  public shared ({ caller }) func addEncryptedMessage(encryptedData : Text, recipientAddress : Text) : async () {
    if (not (AccessControl.hasPermission(accessControlState, caller, #user))) {
      Debug.trap("Unauthorized: Only users can add messages");
    };

    let message : EncryptedMessage = {
      encryptedData;
      recipientAddress;
      created = Time.now();
    };

    let existingMessages = switch (principalMap.get(messages, caller)) {
      case (null) { List.nil<EncryptedMessage>() };
      case (?msgList) { msgList };
    };

    let updatedMessages = List.push(message, existingMessages);
    messages := principalMap.put(messages, caller, updatedMessages);
    addActivityLog(caller, "Message Added", "Message for " # recipientAddress);
  };

  public query ({ caller }) func getEncryptedMessages() : async [EncryptedMessage] {
    if (not (AccessControl.hasPermission(accessControlState, caller, #user))) {
      Debug.trap("Unauthorized: Only users can view messages");
    };

    switch (principalMap.get(messages, caller)) {
      case (null) { [] };
      case (?msgList) { List.toArray(msgList) };
    };
  };

  // Activity Logs
  func addActivityLog(user : Principal, action : Text, details : Text) {
    let log : ActivityLog = {
      timestamp = Time.now();
      action;
      details;
    };

    let existingLogs = switch (principalMap.get(activityLogs, user)) {
      case (null) { List.nil<ActivityLog>() };
      case (?logList) { logList };
    };

    let updatedLogs = List.push(log, existingLogs);
    activityLogs := principalMap.put(activityLogs, user, updatedLogs);
  };

  public query ({ caller }) func getActivityLogs() : async [ActivityLog] {
    if (not (AccessControl.hasPermission(accessControlState, caller, #user))) {
      Debug.trap("Unauthorized: Only users can view activity logs");
    };

    switch (principalMap.get(activityLogs, caller)) {
      case (null) { [] };
      case (?logList) { List.toArray(logList) };
    };
  };

  // Admin Functions
  public query ({ caller }) func getAllVaults() : async [(Principal, Vault)] {
    if (not (AccessControl.hasPermission(accessControlState, caller, #admin))) {
      Debug.trap("Unauthorized: Only admins can view all vaults");
    };

    Iter.toArray(principalMap.entries(vaults));
  };
};
