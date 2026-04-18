namespace Sdkwork.CrawChat.BackendSdk.Api
{
    public static class ApiPaths
    {
        public const string ApiPrefix = "/api/v1";

        public static string BackendPath(string path = "")
        {
            if (string.IsNullOrEmpty(path)) return ApiPrefix;
            if (path.StartsWith("http://") || path.StartsWith("https://")) return path;

            var normalizedPrefix = (ApiPrefix ?? string.Empty).Trim();
            if (!string.IsNullOrEmpty(normalizedPrefix) && normalizedPrefix != "/")
            {
                normalizedPrefix = "/" + normalizedPrefix.Trim('/');
            }
            else
            {
                normalizedPrefix = string.Empty;
            }

            var normalizedPath = path.StartsWith("/") ? path : "/" + path;
            if (string.IsNullOrEmpty(normalizedPrefix)) return normalizedPath;
            if (normalizedPath == normalizedPrefix || normalizedPath.StartsWith(normalizedPrefix + "/")) return normalizedPath;
            return normalizedPrefix + normalizedPath;
        }
    }
}
