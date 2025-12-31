public class Lasagna {
  private static int COOKING_TIME = 40;
  private static int TIME_PER_LAYER = 2;

  public int expectedMinutesInOven() {
    return COOKING_TIME;
  }

  public int remainingMinutesInOven(int timeInOven) {
    return COOKING_TIME - timeInOven;
  }

  public int preparationTimeInMinutes(int layers) {
    return layers * TIME_PER_LAYER;
  }

  public int totalTimeInMinutes(int layers, int timeInOven) {
    return preparationTimeInMinutes(layers) + timeInOven;
  }
}
