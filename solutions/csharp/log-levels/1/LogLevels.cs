static class LogLine
{
    public static string Message(string logLine)
    {
        var parts = logLine.Split(":");
        return parts.Length > 1 ? parts[1].Trim() : string.Empty;
    }

    public static string LogLevel(string logLine)
    {
        var parts = logLine.Split(":");
        return parts.Length > 0 ? new string(parts[0].Trim().ToLower().Where(c => c != '[' && c != ']').ToArray()) : string.Empty;
    }

    public static string Reformat(string logLine)
    {
        var message = Message(logLine);
        var logLevel = LogLevel(logLine);
        return $"{message} ({logLevel})";
    }
}
