import { useState } from 'react';
import { useCreateVault } from '../hooks/useQueries';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';

export default function CreateVaultForm() {
  const [primaryAddress, setPrimaryAddress] = useState('');
  const [backupAddress, setBackupAddress] = useState('');
  const [inactivityPeriod, setInactivityPeriod] = useState('90');
  const { mutate: createVault, isPending } = useCreateVault();

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    createVault({
      primaryAddress,
      backupAddress,
      inactivityPeriod: BigInt(inactivityPeriod),
    });
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <div className="space-y-2">
        <Label htmlFor="primaryAddress">Primary Bitcoin Address</Label>
        <Input
          id="primaryAddress"
          placeholder="bc1q..."
          value={primaryAddress}
          onChange={(e) => setPrimaryAddress(e.target.value)}
          required
        />
        <p className="text-xs text-muted-foreground">
          Your main Bitcoin address managed by the vault
        </p>
      </div>

      <div className="space-y-2">
        <Label htmlFor="backupAddress">Backup Wallet Address (Heir)</Label>
        <Input
          id="backupAddress"
          placeholder="bc1q..."
          value={backupAddress}
          onChange={(e) => setBackupAddress(e.target.value)}
          required
        />
        <p className="text-xs text-muted-foreground">
          Address that receives funds after inactivity period
        </p>
      </div>

      <div className="space-y-2">
        <Label htmlFor="inactivityPeriod">Inactivity Period</Label>
        <Select value={inactivityPeriod} onValueChange={setInactivityPeriod}>
          <SelectTrigger id="inactivityPeriod">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="30">30 Days</SelectItem>
            <SelectItem value="90">90 Days</SelectItem>
            <SelectItem value="180">180 Days</SelectItem>
          </SelectContent>
        </Select>
        <p className="text-xs text-muted-foreground">
          Time before automatic transfer to backup address
        </p>
      </div>

      <Button type="submit" className="w-full" disabled={isPending}>
        {isPending ? 'Creating Vault...' : 'Create Vault'}
      </Button>
    </form>
  );
}
