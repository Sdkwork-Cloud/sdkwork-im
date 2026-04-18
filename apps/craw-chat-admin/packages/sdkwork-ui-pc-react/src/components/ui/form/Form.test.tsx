import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { useForm } from 'react-hook-form';
import { Button } from '../button';
import { Input } from '../input';
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from './form';

interface WorkspaceFormValues {
  name: string;
}

function WorkspaceForm() {
  const form = useForm<WorkspaceFormValues>({
    defaultValues: {
      name: '',
    },
  });
  const handleSubmit = vi.fn();

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(handleSubmit)}>
        <FormField
          control={form.control}
          name="name"
          rules={{ required: 'Workspace name is required.' }}
          render={({ field }) => (
            <FormItem>
              <FormLabel>Workspace Name</FormLabel>
              <FormControl>
                <Input placeholder="SDKWORK Desktop" {...field} />
              </FormControl>
              <FormDescription>Used in the desktop shell and project switchers.</FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
        <Button type="submit">Save</Button>
      </form>
    </Form>
  );
}

describe('Form', () => {
  it('wires label, description, validation message, and control accessibility for desktop forms', async () => {
    const { container } = render(<WorkspaceForm />);

    const input = screen.getByRole('textbox', { name: 'Workspace Name' });

    expect(screen.getByText('Used in the desktop shell and project switchers.')).toBeInTheDocument();
    expect(input).toHaveAttribute('aria-describedby');
    expect(container.querySelector('[data-sdk-ui="field"]')).toHaveAttribute('data-slot', 'field');

    fireEvent.click(screen.getByRole('button', { name: 'Save' }));

    expect(await screen.findByText('Workspace name is required.')).toBeInTheDocument();
    expect(input).toHaveAttribute('aria-invalid', 'true');
  });
});
