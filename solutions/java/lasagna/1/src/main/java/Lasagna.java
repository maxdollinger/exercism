public class Lasagna {
  private static int COOKING_TIME = 40;
  private static int TIME_PER_LAYER = 2;

  // TODO: define the 'expectedMinutesInOven()' method
  public int expectedMinutesInOven() {
    return COOKING_TIME;
  }

  // TODO: define the 'remainingMinutesInOven()' method
  public int remainingMinutesInOven(int timeInOven) {
    return COOKING_TIME - timeInOven;
  }

  // TODO: define the 'preparationTimeInMinutes()' method
  public int preparationTimeInMinutes(int layers) {
    return layers * TIME_PER_LAYER;
  }

  // TODO: define the 'totalTimeInMinutes()' method
  public int totalTimeInMinutes(int layers, int timeInOven) {
    return preparationTimeInMinutes(layers) + timeInOven;
  }
}
