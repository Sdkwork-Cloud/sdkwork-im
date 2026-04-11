import {
  assertPortalSnapshotRecord,
  assertPortalTableItems,
  assertPortalText,
} from '../../../craw-chat-portal-commons/src/index.js';
import { readPortalAuthViewModel } from '../repository/index.js';

export async function buildPortalAuthViewModel() {
  const viewModel = await readPortalAuthViewModel();
  assertPortalSnapshotRecord('Auth view model', viewModel);
  assertPortalText('Auth view model.eyebrow', viewModel.eyebrow);
  assertPortalText('Auth view model.title', viewModel.title);
  assertPortalText('Auth view model.description', viewModel.description);
  assertPortalTableItems('Auth view model details', viewModel.details, ['label', 'value']);
  assertPortalText('Auth view model.primaryActionLabel', viewModel.primaryActionLabel);
  assertPortalText('Auth view model.secondaryActionLabel', viewModel.secondaryActionLabel);
  return viewModel;
}
