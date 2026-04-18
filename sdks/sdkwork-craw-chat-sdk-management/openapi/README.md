# OpenAPI Assets

- `craw-chat-management.openapi.json`
  Checked-in OpenAPI 3.1 authority for the current `/api/admin/*` operator-console backend.
- `craw-chat-management.sdkgen.json`
  Derived sdkgen input. It currently mirrors the authority document until language generation is
  wired.

Refresh both files together with the assembly snapshot:

```bash
node ../bin/materialize-management-authority.mjs
```
