class RemoteControlCar
{
    private uint distanceDriven = 0;
    private uint batteryPercentage = 100;
    public static RemoteControlCar Buy() => new RemoteControlCar();
    public string DistanceDisplay() => $"Driven {distanceDriven} meters";
    public string BatteryDisplay() => batteryPercentage == 0 ? "Battery empty" : $"Battery at {batteryPercentage}%";
    public void Drive()
    {
        if (batteryPercentage > 0)
        {
            batteryPercentage -= 1;
            distanceDriven += 20;
        }
    }
}
