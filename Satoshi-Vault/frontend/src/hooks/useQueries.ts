import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { useActor } from './useActor';
import type { UserProfile, Vault, ActivityLog, EncryptedMessage } from '../backend';
import { toast } from 'sonner';

export function useGetCallerUserProfile() {
  const { actor, isFetching: actorFetching } = useActor();

  const query = useQuery<UserProfile | null>({
    queryKey: ['currentUserProfile'],
    queryFn: async () => {
      if (!actor) throw new Error('Actor not available');
      return actor.getCallerUserProfile();
    },
    enabled: !!actor && !actorFetching,
    retry: false,
  });

  return {
    ...query,
    isLoading: actorFetching || query.isLoading,
    isFetched: !!actor && query.isFetched,
  };
}

export function useSaveCallerUserProfile() {
  const { actor } = useActor();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (profile: UserProfile) => {
      if (!actor) throw new Error('Actor not available');
      return actor.saveCallerUserProfile(profile);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['currentUserProfile'] });
      toast.success('Profile saved successfully');
    },
    onError: (error: Error) => {
      toast.error(error.message || 'Failed to save profile');
    },
  });
}

export function useGetVault() {
  const { actor, isFetching: actorFetching } = useActor();

  return useQuery<Vault | null>({
    queryKey: ['vault'],
    queryFn: async () => {
      if (!actor) return null;
      return actor.getVault();
    },
    enabled: !!actor && !actorFetching,
  });
}

export function useCreateVault() {
  const { actor } = useActor();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async ({
      primaryAddress,
      backupAddress,
      inactivityPeriod,
    }: {
      primaryAddress: string;
      backupAddress: string;
      inactivityPeriod: bigint;
    }) => {
      if (!actor) throw new Error('Actor not available');
      return actor.createVault(primaryAddress, backupAddress, inactivityPeriod);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['vault'] });
      queryClient.invalidateQueries({ queryKey: ['activityLogs'] });
      toast.success('Vault created successfully');
    },
    onError: (error: Error) => {
      toast.error(error.message || 'Failed to create vault');
    },
  });
}

export function useUpdateActivity() {
  const { actor } = useActor();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async () => {
      if (!actor) throw new Error('Actor not available');
      return actor.updateActivity();
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['vault'] });
      queryClient.invalidateQueries({ queryKey: ['activityLogs'] });
      toast.success('Activity updated');
    },
    onError: (error: Error) => {
      toast.error(error.message || 'Failed to update activity');
    },
  });
}

export function useGetActivityLogs() {
  const { actor, isFetching: actorFetching } = useActor();

  return useQuery<ActivityLog[]>({
    queryKey: ['activityLogs'],
    queryFn: async () => {
      if (!actor) return [];
      return actor.getActivityLogs();
    },
    enabled: !!actor && !actorFetching,
  });
}

export function useGetEncryptedMessages() {
  const { actor, isFetching: actorFetching } = useActor();

  return useQuery<EncryptedMessage[]>({
    queryKey: ['encryptedMessages'],
    queryFn: async () => {
      if (!actor) return [];
      return actor.getEncryptedMessages();
    },
    enabled: !!actor && !actorFetching,
  });
}

export function useAddEncryptedMessage() {
  const { actor } = useActor();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async ({
      encryptedData,
      recipientAddress,
    }: {
      encryptedData: string;
      recipientAddress: string;
    }) => {
      if (!actor) throw new Error('Actor not available');
      return actor.addEncryptedMessage(encryptedData, recipientAddress);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['encryptedMessages'] });
      queryClient.invalidateQueries({ queryKey: ['activityLogs'] });
      toast.success('Message added successfully');
    },
    onError: (error: Error) => {
      toast.error(error.message || 'Failed to add message');
    },
  });
}
