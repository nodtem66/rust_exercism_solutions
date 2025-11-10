using System;
public class Player
{
    public static Random random = new Random();
    public int RollDie() => random.Next(1, 19);
    public double GenerateSpellStrength() => random.NextDouble() * 100;
}
