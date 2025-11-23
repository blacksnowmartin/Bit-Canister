import { useGetVault, useGetCallerUserProfile } from '../hooks/useQueries';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import VaultOverview from '../components/VaultOverview';
import CreateVaultForm from '../components/CreateVaultForm';
import ActivityLogsList from '../components/ActivityLogsList';
import EncryptedMessagesList from '../components/EncryptedMessagesList';
import { Shield } from 'lucide-react';

export default function Dashboard() {
  const { data: vault, isLoading: vaultLoading } = useGetVault();
  const { data: userProfile } = useGetCallerUserProfile();

  if (vaultLoading) {
    return (
      <div className="container flex min-h-[calc(100vh-8rem)] items-center justify-center py-12">
        <div className="text-center">
          <div className="mb-4 h-12 w-12 animate-spin rounded-full border-4 border-primary border-t-transparent mx-auto" />
          <p className="text-muted-foreground">Loading vault...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="container py-8">
      {/* Welcome Header */}
      <div className="mb-8">
        <h1 className="text-3xl font-bold mb-2">
          Welcome back, {userProfile?.name || 'User'}
        </h1>
        <p className="text-muted-foreground">
          Manage your Bitcoin inheritance vault and secure your legacy
        </p>
      </div>

      {!vault ? (
        <div className="grid gap-8 lg:grid-cols-2">
          <div>
            <Card className="border-2 border-orange-500/20">
              <CardHeader>
                <div className="mb-4 flex h-12 w-12 items-center justify-center rounded-lg bg-gradient-to-br from-orange-500 to-amber-600">
                  <Shield className="h-6 w-6 text-white" />
                </div>
                <CardTitle>Create Your Vault</CardTitle>
                <CardDescription>
                  Set up your Bitcoin inheritance vault with automatic transfer capabilities
                </CardDescription>
              </CardHeader>
              <CardContent>
                <CreateVaultForm />
              </CardContent>
            </Card>
          </div>
          <div>
            <Card>
              <CardHeader>
                <CardTitle>How Your Vault Works</CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div>
                  <h4 className="font-medium mb-1">1. Primary Address</h4>
                  <p className="text-sm text-muted-foreground">
                    Your main Bitcoin address managed on-chain by the vault
                  </p>
                </div>
                <div>
                  <h4 className="font-medium mb-1">2. Backup Address</h4>
                  <p className="text-sm text-muted-foreground">
                    The heir's address that receives funds after inactivity period
                  </p>
                </div>
                <div>
                  <h4 className="font-medium mb-1">3. Inactivity Period</h4>
                  <p className="text-sm text-muted-foreground">
                    Time before automatic transfer (30, 90, or 180 days)
                  </p>
                </div>
                <div>
                  <h4 className="font-medium mb-1">4. Stay Active</h4>
                  <p className="text-sm text-muted-foreground">
                    Regular activity resets the countdown timer
                  </p>
                </div>
              </CardContent>
            </Card>
          </div>
        </div>
      ) : (
        <Tabs defaultValue="overview" className="space-y-6">
          <TabsList>
            <TabsTrigger value="overview">Overview</TabsTrigger>
            <TabsTrigger value="activity">Activity Logs</TabsTrigger>
            <TabsTrigger value="messages">Encrypted Messages</TabsTrigger>
          </TabsList>

          <TabsContent value="overview" className="space-y-6">
            <VaultOverview vault={vault} />
          </TabsContent>

          <TabsContent value="activity" className="space-y-6">
            <ActivityLogsList />
          </TabsContent>

          <TabsContent value="messages" className="space-y-6">
            <EncryptedMessagesList />
          </TabsContent>
        </Tabs>
      )}
    </div>
  );
}
