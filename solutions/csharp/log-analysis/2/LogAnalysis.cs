public static class LogAnalysis
{
    public static string SubstringAfter(this string str, string delimiter)
    {
        int index = str.IndexOf(delimiter);
        return index == -1 ? "" : str.Substring(index + delimiter.Length);
    }
    public static string SubstringBetween(this string str, string startDelimiter, string endDelimiter)
    {
        int startIndex = str.IndexOf(startDelimiter);
        if (startIndex == -1) return "";

        int endIndex = str.IndexOf(endDelimiter);
        if (endIndex == -1) return "";

        return str.Substring(startIndex + startDelimiter.Length, endIndex - (startIndex + startDelimiter.Length));
    }
    public static string Message(this string logLine) => logLine.SubstringAfter(": ");
    public static string LogLevel(this string logLine) => logLine.SubstringBetween("[", "]");
}