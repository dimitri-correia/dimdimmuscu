function brzycki(weight: number, reps: number): number {
  return weight * (36 / (37 - reps));
}

function brzycki2(weight: number, reps: number): number {
  return weight / (1.0278 - 0.0278 * reps);
}

function epley(weight: number, reps: number): number {
  return weight * (1 + 0.0333 * reps);
}

function lander(weight: number, reps: number): number {
  return (100 * weight) / (101.3 - 2.67123 * reps);
}

function lombardi(weight: number, reps: number): number {
  return weight * reps ** 0.1;
}

function mayhew(weight: number, reps: number): number {
  return (100 * weight) / (52.2 + 41.9 * Math.exp(-0.055 * reps));
}

function oconnor(weight: number, reps: number): number {
  return weight * (1 + 0.025 * reps);
}

function wathan(weight: number, reps: number): number {
  return (100 * weight) / (48.8 + 53.8 * Math.exp(-0.075 * reps));
}
