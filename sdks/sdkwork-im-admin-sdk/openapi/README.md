# OpenAPI Assets

- `im-admin.openapi.json`
  Checked-in OpenAPI 3.1 authority for the current `/api/admin/*` operator-console backend.
- `im-admin.sdkgen.json`
  Derived sdkgen input. It currently mirrors the authority document until language generation is
  wired.

Refresh both files together with the assembly snapshot:

```bash
node ../bin/materialize-im-admin-authority.mjs
```
