import { addCardioEntry, getCardioEntries } from "../database/CardioDB";

export interface CardioEntry {
  id: number;
  date: Date;
  name: string;
  time: number;
  calo: number;
}

export function refreshWeightEntries(
  setCardioEntries: (
    value: ((prevState: CardioEntry[]) => CardioEntry[]) | CardioEntry[]
  ) => void
) {
  getCardioEntries()
    .then((ce: CardioEntry[]) => {
      setCardioEntries(ce);
    })
    .catch(() => console.debug("error fetching cardio entries"));
}

export function addWeight(name: string, time: number, calo: number) {
  const today: string = new Date().toISOString().split("T")[0];
  addCardioEntry(today, name, time, calo);
}
