import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Shield, Clock, Lock, Key } from 'lucide-react';
import { useInternetIdentity } from '../hooks/useInternetIdentity';

export default function LandingPage() {
  const { login, loginStatus } = useInternetIdentity();

  return (
    <div className="container py-12">
      {/* Hero Section */}
      <section className="mb-20 text-center">
        <div className="mx-auto mb-8 flex h-24 w-24 items-center justify-center rounded-2xl bg-gradient-to-br from-orange-500 to-amber-600 shadow-lg">
          <Shield className="h-12 w-12 text-white" />
        </div>
        <h1 className="mb-4 text-4xl font-bold tracking-tight sm:text-5xl md:text-6xl">
          Secure Your Bitcoin Legacy
        </h1>
        <p className="mx-auto mb-8 max-w-2xl text-lg text-muted-foreground">
          A fully on-chain Bitcoin inheritance and recovery vault powered by the Internet Computer.
          Protect your assets with automatic transfers and encrypted messages for your heirs.
        </p>
        <Button size="lg" onClick={login} disabled={loginStatus === 'logging-in'}>
          {loginStatus === 'logging-in' ? 'Connecting...' : 'Get Started'}
        </Button>
      </section>

      {/* Hero Image */}
      <section className="mb-20">
        <div className="overflow-hidden rounded-2xl border border-border/40 bg-muted/50">
          <img
            src="/assets/generated/dashboard-hero.dim_1200x600.png"
            alt="Satoshi Vault Dashboard"
            className="w-full h-auto"
          />
        </div>
      </section>

      {/* Features */}
      <section className="mb-20">
        <h2 className="mb-12 text-center text-3xl font-bold">How It Works</h2>
        <div className="grid gap-8 md:grid-cols-2 lg:grid-cols-4">
          <Card>
            <CardHeader>
              <div className="mb-4 flex h-12 w-12 items-center justify-center rounded-lg bg-orange-500/10">
                <Shield className="h-6 w-6 text-orange-500" />
              </div>
              <CardTitle>Create Your Vault</CardTitle>
              <CardDescription>
                Set up your Bitcoin vault with primary and backup wallet addresses
              </CardDescription>
            </CardHeader>
          </Card>

          <Card>
            <CardHeader>
              <div className="mb-4 flex h-12 w-12 items-center justify-center rounded-lg bg-amber-500/10">
                <Clock className="h-6 w-6 text-amber-500" />
              </div>
              <CardTitle>Set Inactivity Period</CardTitle>
              <CardDescription>
                Choose 30, 90, or 180 days as your inactivity threshold
              </CardDescription>
            </CardHeader>
          </Card>

          <Card>
            <CardHeader>
              <div className="mb-4 flex h-12 w-12 items-center justify-center rounded-lg bg-orange-600/10">
                <Lock className="h-6 w-6 text-orange-600" />
              </div>
              <CardTitle>Automatic Transfer</CardTitle>
              <CardDescription>
                ckBTC automatically transfers to your backup wallet if inactive
              </CardDescription>
            </CardHeader>
          </Card>

          <Card>
            <CardHeader>
              <div className="mb-4 flex h-12 w-12 items-center justify-center rounded-lg bg-amber-600/10">
                <Key className="h-6 w-6 text-amber-600" />
              </div>
              <CardTitle>Encrypted Messages</CardTitle>
              <CardDescription>
                Leave encrypted instructions for your heirs using vetKeys
              </CardDescription>
            </CardHeader>
          </Card>
        </div>
      </section>

      {/* Security Features */}
      <section className="mb-20">
        <div className="grid gap-8 lg:grid-cols-2 items-center">
          <div>
            <h2 className="mb-4 text-3xl font-bold">Built on Cutting-Edge Technology</h2>
            <ul className="space-y-4">
              <li className="flex gap-3">
                <div className="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-orange-500/10">
                  <div className="h-2 w-2 rounded-full bg-orange-500" />
                </div>
                <div>
                  <p className="font-medium">Internet Computer Protocol</p>
                  <p className="text-sm text-muted-foreground">
                    Fully on-chain execution with no external custodians
                  </p>
                </div>
              </li>
              <li className="flex gap-3">
                <div className="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-orange-500/10">
                  <div className="h-2 w-2 rounded-full bg-orange-500" />
                </div>
                <div>
                  <p className="font-medium">ckBTC Integration</p>
                  <p className="text-sm text-muted-foreground">
                    Native Bitcoin balance management on-chain
                  </p>
                </div>
              </li>
              <li className="flex gap-3">
                <div className="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-orange-500/10">
                  <div className="h-2 w-2 rounded-full bg-orange-500" />
                </div>
                <div>
                  <p className="font-medium">Threshold ECDSA</p>
                  <p className="text-sm text-muted-foreground">
                    Secure Bitcoin transaction signing without private key exposure
                  </p>
                </div>
              </li>
              <li className="flex gap-3">
                <div className="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-orange-500/10">
                  <div className="h-2 w-2 rounded-full bg-orange-500" />
                </div>
                <div>
                  <p className="font-medium">vetKeys Encryption</p>
                  <p className="text-sm text-muted-foreground">
                    End-to-end encrypted messages for your beneficiaries
                  </p>
                </div>
              </li>
            </ul>
          </div>
          <div className="flex justify-center">
            <img
              src="/assets/generated/security-shield-transparent.dim_200x200.png"
              alt="Security Shield"
              className="h-64 w-64"
            />
          </div>
        </div>
      </section>

      {/* CTA */}
      <section className="text-center">
        <Card className="border-2 border-orange-500/20 bg-gradient-to-br from-orange-500/5 to-amber-600/5">
          <CardContent className="py-12">
            <h2 className="mb-4 text-3xl font-bold">Ready to Secure Your Bitcoin?</h2>
            <p className="mx-auto mb-8 max-w-2xl text-muted-foreground">
              Join Satoshi Vault today and ensure your Bitcoin legacy is protected with the most
              advanced on-chain inheritance system.
            </p>
            <Button size="lg" onClick={login} disabled={loginStatus === 'logging-in'}>
              {loginStatus === 'logging-in' ? 'Connecting...' : 'Create Your Vault'}
            </Button>
          </CardContent>
        </Card>
      </section>
    </div>
  );
}
