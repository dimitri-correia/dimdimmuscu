export interface PersonalRecordEntry {
  id: number;
  date: Date;
  exId: number;
  weightLifted: number;
}

export function getImprovementString(newWeight: number, oldWeight: number) {
  console.log(newWeight);
  console.log(oldWeight);
  return `${newWeight - oldWeight} (${(newWeight - oldWeight) / newWeight}%)`;
}
