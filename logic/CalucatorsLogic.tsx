export const calculateOneRM = (weight: number, reps: number) => {
  const brzycki = weight / (1.0278 - 0.0278 * reps);
  const brzycki2 = weight * (36 / (37 - reps));
  const epley = weight * (1 + reps / 30);
  const lander = (100 * weight) / (101.3 - 2.67123 * reps);
  const lombardi = weight * reps ** 0.1;
  const mayhew = (100 * weight) / (52.2 + 41.9 * Math.exp(-0.055 * reps));
  const oconnor = weight * (1 + 0.025 * reps);
  const wathan = (100 * weight) / (48.8 + 53.8 * Math.exp(-0.075 * reps));

  return {
    Brzycki: brzycki,
    Brzycki2: brzycki2,
    Epley: epley,
    Lander: lander,
    Lombardi: lombardi,
    Mayhew: mayhew,
    OConnor: oconnor,
    Wathan: wathan,
  };
};
