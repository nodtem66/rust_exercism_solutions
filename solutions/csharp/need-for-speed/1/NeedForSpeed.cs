class RemoteControlCar
{
    private readonly int speed = 0;
    private readonly int batteryDrain = 0;
    private int distanceDriven = 0;
    private int batteryLeft = 100;
    public RemoteControlCar(int speed, int batteryDrain)
    {
        this.speed = speed;
        this.batteryDrain = batteryDrain;
    }

    public bool BatteryDrained() => batteryLeft < batteryDrain;

    public int DistanceDriven() => distanceDriven;

    public void Drive()
    {
        if (batteryLeft >= batteryDrain)
        {
            distanceDriven += speed;
            batteryLeft -= batteryDrain;
        }
    }

    public static RemoteControlCar Nitro() => new RemoteControlCar(50, 4);
}

class RaceTrack
{
    private readonly int distance;
    public RaceTrack(int distance) => this.distance = distance;
    public bool TryFinishTrack(RemoteControlCar car)
    {
        while (!car.BatteryDrained())
        {
            car.Drive();
            if (car.DistanceDriven() >= this.distance)
            {
                return true;
            }
        }
        return false;
    }
}
