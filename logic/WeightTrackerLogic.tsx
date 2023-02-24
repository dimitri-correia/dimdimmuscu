export interface WeightEntry {
  id: number;
  date: Date;
  weight: number;
}

export const averageXdays = (
  entries: WeightEntry[],
  date: Date,
  numberOfDays: number
): number => {
  const entriesInLastXDays: number[] = entries
    .filter(
      (entry) =>
        entry.date <= date &&
        entry.date >
          new Date(new Date().setDate(new Date().getDate() - numberOfDays))
    )
    .map((entry) => entry.weight);
  return (
    entriesInLastXDays.reduce((acc, val) => acc + val, 0) /
    entriesInLastXDays.length
  );
};
