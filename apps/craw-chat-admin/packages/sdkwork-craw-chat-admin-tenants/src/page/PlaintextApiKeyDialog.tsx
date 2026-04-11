import {
  Button,
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  InlineAlert,
} from '@sdkwork/ui-pc-react';
import { useAdminI18n } from 'sdkwork-craw-chat-admin-core';

import { copyToClipboard, type RevealedApiKey } from './shared';

type PlaintextApiKeyDialogProps = {
  onClose: () => void;
  revealedApiKey: RevealedApiKey;
};

export function PlaintextApiKeyDialog({
  onClose,
  revealedApiKey,
}: PlaintextApiKeyDialogProps) {
  const { t } = useAdminI18n();

  return (
    <Dialog
      onOpenChange={(open: boolean) => !open && onClose()}
      open={Boolean(revealedApiKey)}
    >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t('Plaintext key ready')}</DialogTitle>
          <DialogDescription>
            {t(
              'Copy this live access secret now. It is only revealed during this operator handoff window.',
            )}
          </DialogDescription>
        </DialogHeader>
        {revealedApiKey ? (
          <div className="space-y-4">
            <InlineAlert
              description={`${revealedApiKey.project_id} | ${revealedApiKey.environment} | ${revealedApiKey.label || t('Unlabeled key')}`}
              showIcon
              title={t('Operator handoff')}
              tone="info"
            />
            <Card>
              <CardHeader>
                <CardTitle className="text-base">{t('Plaintext key')}</CardTitle>
                <CardDescription>
                  {t('Copy the key now. It will not be revealed again in this dialog.')}
                </CardDescription>
              </CardHeader>
              <CardContent className="break-all font-mono text-sm">
                {revealedApiKey.plaintext}
              </CardContent>
            </Card>
            <div className="flex flex-wrap items-center justify-end gap-2">
              <Button
                onClick={() => void copyToClipboard(revealedApiKey.plaintext)}
                type="button"
                variant="primary"
              >
                {t('Copy key')}
              </Button>
              <Button onClick={onClose} type="button" variant="outline">
                {t('Close')}
              </Button>
            </div>
          </div>
        ) : null}
      </DialogContent>
    </Dialog>
  );
}
