import { calculateOneRM } from "../logic/CalucatorsLogic";

describe("calculateOneRM", () => {
  it("should return the correct values for given weight and reps", () => {
    const result = calculateOneRM(100, 5);
    expect(result.Brzycki).toBeCloseTo(112.5, 0.1);
    expect(result.Brzycki2).toBeCloseTo(112.5, 0.1);
    expect(result.Epley).toBeCloseTo(116.7, 0.1);
    expect(result.Lander).toBeCloseTo(113.7, 0.1);
    expect(result.Lombardi).toBeCloseTo(117.4, 0.1);
    expect(result.Mayhew).toBeCloseTo(119.0, 0.1);
    expect(result.OConnor).toBeCloseTo(112.5, 0.1);
    expect(result.Wathan).toBeCloseTo(116.6, 0.1);
  });
});
