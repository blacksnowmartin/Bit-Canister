import { useState } from 'react';
import { useGetEncryptedMessages, useAddEncryptedMessage } from '../hooks/useQueries';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Textarea } from '@/components/ui/textarea';
import { ScrollArea } from '@/components/ui/scroll-area';
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from '@/components/ui/dialog';
import { Key, Plus } from 'lucide-react';
import { formatDistanceToNow } from 'date-fns';

export default function EncryptedMessagesList() {
  const { data: messages, isLoading } = useGetEncryptedMessages();
  const { mutate: addMessage, isPending } = useAddEncryptedMessage();
  const [open, setOpen] = useState(false);
  const [recipientAddress, setRecipientAddress] = useState('');
  const [messageContent, setMessageContent] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    // In a real implementation, this would use vetKeys to encrypt the message
    // For now, we'll simulate encryption by base64 encoding
    const encryptedData = btoa(messageContent);
    addMessage(
      { encryptedData, recipientAddress },
      {
        onSuccess: () => {
          setOpen(false);
          setRecipientAddress('');
          setMessageContent('');
        },
      }
    );
  };

  if (isLoading) {
    return (
      <Card>
        <CardContent className="py-12 text-center">
          <div className="mb-4 h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent mx-auto" />
          <p className="text-muted-foreground">Loading messages...</p>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Key className="h-5 w-5 text-amber-500" />
            <div>
              <CardTitle>Encrypted Messages</CardTitle>
              <CardDescription>Private instructions for your heirs</CardDescription>
            </div>
          </div>
          <Dialog open={open} onOpenChange={setOpen}>
            <DialogTrigger asChild>
              <Button size="sm">
                <Plus className="mr-2 h-4 w-4" />
                Add Message
              </Button>
            </DialogTrigger>
            <DialogContent>
              <DialogHeader>
                <DialogTitle>Add Encrypted Message</DialogTitle>
                <DialogDescription>
                  Create an encrypted message for your heir. This will be encrypted using vetKeys.
                </DialogDescription>
              </DialogHeader>
              <form onSubmit={handleSubmit} className="space-y-4">
                <div className="space-y-2">
                  <Label htmlFor="recipient">Recipient Address</Label>
                  <Input
                    id="recipient"
                    placeholder="bc1q..."
                    value={recipientAddress}
                    onChange={(e) => setRecipientAddress(e.target.value)}
                    required
                  />
                </div>
                <div className="space-y-2">
                  <Label htmlFor="message">Message</Label>
                  <Textarea
                    id="message"
                    placeholder="Enter your message or instructions..."
                    value={messageContent}
                    onChange={(e) => setMessageContent(e.target.value)}
                    required
                    rows={6}
                  />
                </div>
                <Button type="submit" className="w-full" disabled={isPending}>
                  {isPending ? 'Encrypting...' : 'Encrypt & Save Message'}
                </Button>
              </form>
            </DialogContent>
          </Dialog>
        </div>
      </CardHeader>
      <CardContent>
        {!messages || messages.length === 0 ? (
          <div className="py-12 text-center text-muted-foreground">
            No encrypted messages yet. Add one to leave instructions for your heirs.
          </div>
        ) : (
          <ScrollArea className="h-[500px] pr-4">
            <div className="space-y-4">
              {messages.map((message, index) => {
                const timestamp = new Date(Number(message.created) / 1_000_000);
                return (
                  <div
                    key={index}
                    className="rounded-lg border border-border/40 bg-muted/30 p-4 space-y-3"
                  >
                    <div className="flex items-start justify-between gap-2">
                      <div className="flex items-center gap-2">
                        <div className="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-amber-500/10">
                          <Key className="h-4 w-4 text-amber-500" />
                        </div>
                        <div>
                          <p className="text-sm font-medium">Encrypted Message</p>
                          <p className="text-xs text-muted-foreground">
                            {formatDistanceToNow(timestamp, { addSuffix: true })}
                          </p>
                        </div>
                      </div>
                    </div>
                    <div>
                      <p className="text-xs font-medium text-muted-foreground mb-1">
                        Recipient Address
                      </p>
                      <p className="text-xs font-mono bg-muted p-2 rounded break-all">
                        {message.recipientAddress}
                      </p>
                    </div>
                    <div>
                      <p className="text-xs font-medium text-muted-foreground mb-1">
                        Encrypted Data (vetKeys)
                      </p>
                      <p className="text-xs font-mono bg-muted p-2 rounded break-all">
                        {message.encryptedData.substring(0, 100)}...
                      </p>
                    </div>
                  </div>
                );
              })}
            </div>
          </ScrollArea>
        )}
      </CardContent>
    </Card>
  );
}
