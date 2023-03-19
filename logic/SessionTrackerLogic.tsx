import {
  addSessionTrackerEntry,
  getSessionTrackerEntries,
} from "../database/SessionTrackerDB";

export interface SessionTrackerEntry {
  id: number;
  date: Date;
  name: string;
  set: number;
  rep: number;
  weight: number;
}

export function refreshSessionTrackerEntries(
  setCardioEntries: (
    value:
      | ((prevState: SessionTrackerEntry[]) => SessionTrackerEntry[])
      | SessionTrackerEntry[]
  ) => void
) {
  getSessionTrackerEntries()
    .then((ce: SessionTrackerEntry[]) => {
      setCardioEntries(ce);
    })
    .catch(() => console.debug("error fetching Session Tracker entries"));
}

export function addSessionTracker(name: string, time: string, calo: string) {
  const today: string = new Date().toISOString().split("T")[0];
  const timeN = parseInt(time);
  const caloN = parseInt(calo);
  addSessionTrackerEntry(today, name, timeN, caloN);
}
