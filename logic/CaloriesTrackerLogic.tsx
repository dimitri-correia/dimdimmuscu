const MIN_HEALTHY_BMI = 18.5;
const MAX_HEALTHY_BMI = 25;
const WATER_INTAKE_FACTOR = 0.033;

export enum ActivityLevels {
  SEDENTARY = 1.2,
  LIGHTLY_ACTIVE = 1.375,
  MODERATELY_ACTIVE = 1.55,
  VERY_ACTIVE = 1.725,
  EXTREMELY_ACTIVE = 1.9,
}

export interface CaloEntry {
  id: number;
  date: Date;
  name: string;
  calo: number;
  prot: number;
}

export const calculatePercentage = (value: number, goal: number) => {
  return ((value / goal) * 100).toFixed(2);
};

export function calculateWeightAndBMIStats(
  age: number,
  weight: number,
  height: number
): {
  bmi: number;
  maxHealthyWeight: number;
  minHealthyWeight: number;
} {
  const heightInMeters = height / 100; // convert height from cm to m
  const bmi = weight / (heightInMeters * heightInMeters); // calculate BMI

  // calculate healthy BMI range
  const minHealthyWeight = MIN_HEALTHY_BMI * heightInMeters * heightInMeters;
  const maxHealthyWeight = MAX_HEALTHY_BMI * heightInMeters * heightInMeters;

  return {
    bmi,
    maxHealthyWeight,
    minHealthyWeight,
  };
}

export function calculateWaterIntake(weight: number): number {
  return weight * WATER_INTAKE_FACTOR;
}

export function calculateBMR(
  weight: number,
  height: number,
  age: number,
  isMale: boolean
): { mifflinStJeor: number; martinBerkhan: number } {
  const s = isMale ? 5 : -161;
  const bmrMifflinStJeor = 10 * weight + 6.25 * height - 5 * age + s;
  const bmrMartinBerkhan =
    370 + 21.6 * (weight * (1 - 0.01 * weight)) + 6.3 * height - 3.6 * age + s;
  return { mifflinStJeor: bmrMifflinStJeor, martinBerkhan: bmrMartinBerkhan };
}

export function calculateMuscularPotential(
  weight: number,
  bodyFatPercentage: number
): Record<number, number> {
  const leanBodyMass = weight * (1 - bodyFatPercentage / 100); // calculate lean body mass
  const bmr = calculateBMR(leanBodyMass, 175, 30, true).martinBerkhan; // calculate BMR using Martin Berkhan's formula
  const maxMuscleMass = bmr / 24; // calculate maximum muscle mass
  const muscleMassAt5PercentBF = maxMuscleMass * 0.95; // calculate muscle mass at 5% body fat
  const muscleMassAt10PercentBF = maxMuscleMass * 0.9; // calculate muscle mass at 10% body fat
  const muscleMassAt15PercentBF = maxMuscleMass * 0.85; // calculate muscle mass at 15% body fat
  const goalWeights = [
    muscleMassAt5PercentBF,
    muscleMassAt10PercentBF,
    muscleMassAt15PercentBF,
  ].map((muscleMass) => Math.round(muscleMass + (weight - leanBodyMass))); // calculate goal weights for each body fat percentage
  return { 5: goalWeights[0], 10: goalWeights[1], 15: goalWeights[2] };
}
