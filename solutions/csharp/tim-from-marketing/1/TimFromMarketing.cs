static class Badge
{
    public static string Print(int? id, string name, string? department)
    {
        var printedDepartment = department?.ToUpper() ?? "OWNER";
        return id.HasValue ? $"[{id}] - {name} - {printedDepartment}" : $"{name} - {printedDepartment}";
    }
}
