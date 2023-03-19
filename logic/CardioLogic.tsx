import { addCardioEntry, getCardioEntries } from "../database/CardioDB";

export interface CardioEntry {
  id: number;
  date: Date;
  name: string;
  time: number;
  calo: number;
}

export function refreshCardioEntries(
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

export function addCardio(name: string, time: string, calo: string) {
  const today: string = new Date().toISOString().split("T")[0];
  const timeN = parseInt(time);
  const caloN = parseInt(calo);
  addCardioEntry(today, name, timeN, caloN);
}
