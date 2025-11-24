import { useUpdateActivity } from '../hooks/useQueries';
import type { Vault } from '../backend';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Progress } from '@/components/ui/progress';
import { Wallet, Clock, Shield, Activity } from 'lucide-react';
import { formatDistanceToNow } from 'date-fns';

interface VaultOverviewProps {
  vault: Vault;
}

export default function VaultOverview({ vault }: VaultOverviewProps) {
  const { mutate: updateActivity, isPending } = useUpdateActivity();

  const lastActiveDate = new Date(Number(vault.lastActive) / 1_000_000);
  const createdDate = new Date(Number(vault.created) / 1_000_000);
  const inactivityPeriodMs = Number(vault.inactivityPeriod) * 24 * 60 * 60 * 1000;
  const transferDate = new Date(lastActiveDate.getTime() + inactivityPeriodMs);
  const now = new Date();
  const timeRemaining = transferDate.getTime() - now.getTime();
  const daysRemaining = Math.max(0, Math.ceil(timeRemaining / (24 * 60 * 60 * 1000)));
  const progressPercentage = Math.max(
    0,
    Math.min(100, ((inactivityPeriodMs - timeRemaining) / inactivityPeriodMs) * 100)
  );

  return (
    <div className="space-y-6">
      {/* Status Card */}
      <Card className="border-2 border-orange-500/20 bg-gradient-to-br from-orange-500/5 to-amber-600/5">
        <CardHeader>
          <div className="flex items-center justify-between">
            <div>
              <CardTitle className="text-2xl">Vault Status</CardTitle>
              <CardDescription>Your Bitcoin inheritance vault is active</CardDescription>
            </div>
            <Badge variant="outline" className="bg-green-500/10 text-green-600 border-green-500/20">
              Active
            </Badge>
          </div>
        </CardHeader>
        <CardContent className="space-y-6">
          <div>
            <div className="mb-2 flex items-center justify-between text-sm">
              <span className="text-muted-foreground">Time until transfer</span>
              <span className="font-medium">{daysRemaining} days remaining</span>
            </div>
            <Progress value={progressPercentage} className="h-2" />
            <p className="mt-2 text-xs text-muted-foreground">
              Transfer scheduled for {transferDate.toLocaleDateString()}
            </p>
          </div>

          <Button onClick={() => updateActivity()} disabled={isPending} className="w-full">
            <Activity className="mr-2 h-4 w-4" />
            {isPending ? 'Updating...' : 'Update Activity (Reset Timer)'}
          </Button>
        </CardContent>
      </Card>

      {/* Vault Details */}
      <div className="grid gap-6 md:grid-cols-2">
        <Card>
          <CardHeader>
            <div className="flex items-center gap-2">
              <Wallet className="h-5 w-5 text-orange-500" />
              <CardTitle>Addresses</CardTitle>
            </div>
          </CardHeader>
          <CardContent className="space-y-4">
            <div>
              <p className="text-sm font-medium mb-1">Primary Address</p>
              <p className="text-xs text-muted-foreground break-all font-mono bg-muted p-2 rounded">
                {vault.primaryAddress}
              </p>
            </div>
            <div>
              <p className="text-sm font-medium mb-1">Backup Address (Heir)</p>
              <p className="text-xs text-muted-foreground break-all font-mono bg-muted p-2 rounded">
                {vault.backupAddress}
              </p>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <div className="flex items-center gap-2">
              <Clock className="h-5 w-5 text-amber-500" />
              <CardTitle>Timeline</CardTitle>
            </div>
          </CardHeader>
          <CardContent className="space-y-4">
            <div>
              <p className="text-sm font-medium mb-1">Inactivity Period</p>
              <p className="text-2xl font-bold">{vault.inactivityPeriod.toString()} Days</p>
            </div>
            <div>
              <p className="text-sm font-medium mb-1">Last Active</p>
              <p className="text-sm text-muted-foreground">
                {formatDistanceToNow(lastActiveDate, { addSuffix: true })}
              </p>
            </div>
            <div>
              <p className="text-sm font-medium mb-1">Vault Created</p>
              <p className="text-sm text-muted-foreground">
                {formatDistanceToNow(createdDate, { addSuffix: true })}
              </p>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Balance Card */}
      <Card>
        <CardHeader>
          <div className="flex items-center gap-2">
            <Shield className="h-5 w-5 text-orange-600" />
            <CardTitle>ckBTC Balance</CardTitle>
          </div>
        </CardHeader>
        <CardContent>
          <div className="text-3xl font-bold">
            {(Number(vault.ckBTCBalance) / 100_000_000).toFixed(8)} ckBTC
          </div>
          <p className="text-sm text-muted-foreground mt-1">
            Secured on-chain with Threshold ECDSA
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
