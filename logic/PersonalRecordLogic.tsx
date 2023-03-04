export interface PersonalRecordEntry {
  id: number;
  date: Date;
  exId: number;
  weightLifted: number;
}

export function getImprovementString(newWeight: number, oldWeight: number) {
  if (!oldWeight) {
    return "";
  }
  const improvement = (newWeight - oldWeight).toFixed(1);
  const percentage = ((100 * (newWeight - oldWeight)) / oldWeight).toFixed(0);

  return `+ ${improvement} (${percentage}%)`;
}
