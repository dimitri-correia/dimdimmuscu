import {
  addWeightEntry,
  deleteWeightEntry,
  editWeightEntry,
  getWeightEntries,
} from "../database/WeightTrackerDB";
import * as TextWT from "../assets/texts/WeightTracker";
import { confirmationChanges } from "../components/commons/ValidateChanges";

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

export function refreshWeightEntries(
  setWeightEntries: (
    value: ((prevState: WeightEntry[]) => WeightEntry[]) | WeightEntry[]
  ) => void
) {
  getWeightEntries()
    .then((we: WeightEntry[]) => {
      setWeightEntries(we);
    })
    .catch(() => console.debug("error fetching weight entries"));
}

export function editEntry(
  modifiedWeight: string,
  editId: number,
  setEditId: (value: ((prevState: number) => number) | number) => void
) {
  const parsedWeight = parseFloat(modifiedWeight);
  if (isNaN(parsedWeight)) {
    alert(TextWT.invalidWeight);
    return;
  }
  confirmationChanges(() => {
    editWeightEntry(editId, parsedWeight);
    setEditId(-1);
  });
}

export function deleteEntry(
  editId: number,
  setEditId: (value: ((prevState: number) => number) | number) => void
) {
  confirmationChanges(() => {
    deleteWeightEntry(editId);
    setEditId(-1);
  });
}

export function addWeight(weightEntries: WeightEntry[], newWeight: string) {
  const today: string = new Date().toISOString().split("T")[0];
  const hasEntryForToday: boolean = weightEntries.some(
    (entry: WeightEntry) => entry.date.toISOString().split("T")[0] === today
  );

  if (hasEntryForToday) {
    alert(TextWT.alreadyExistingWeight);
  }

  const parsedWeight = parseFloat(newWeight);
  if (isNaN(parsedWeight)) {
    alert(TextWT.invalidWeight);
    return;
  }
  addWeightEntry(today, parsedWeight);
}
