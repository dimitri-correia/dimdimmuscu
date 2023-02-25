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
          new Date(date.getTime() - numberOfDays * 24 * 60 * 60 * 1000)
    )
    .map((entry) => entry.weight);
  return (
    entriesInLastXDays.reduce((acc, val) => acc + val, 0) /
    entriesInLastXDays.length
  );
};
