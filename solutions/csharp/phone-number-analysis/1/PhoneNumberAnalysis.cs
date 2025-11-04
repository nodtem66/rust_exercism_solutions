using System.Diagnostics;

public static class PhoneNumber
{
    public static (bool IsNewYork, bool IsFake, string LocalNumber) Analyze(string phoneNumber)
    {
        // Assume phoneNumber is in the format "XXX-XXX-XXXX"
        var cleanedNumber = phoneNumber.Replace("-", "");
        Debug.Assert(cleanedNumber.Length == 10, "Phone number must be in the format XXX-XXX-XXXX");
        return (cleanedNumber.StartsWith("212"), cleanedNumber[3..6] == "555", cleanedNumber[6..]);
    }

    public static bool IsFake((bool IsNewYork, bool IsFake, string LocalNumber) phoneNumberInfo) => phoneNumberInfo.IsFake;
}
