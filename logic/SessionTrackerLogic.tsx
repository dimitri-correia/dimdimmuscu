import {
  addSessionTrackerEntry,
  getSessionTrackerEntries,
} from "../database/SessionTrackerDB";

export interface SessionTrackerLiftEntry {
  id: number;
  date: Date;
  name: string;
}

export interface SessionTrackerSetEntry {
  id: number;
  idLift: number;
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

export function addSessionTracker(
  name: string,
  set: string,
  rep: string,
  weight: string
) {
  const today: string = new Date().toISOString().split("T")[0];
  const setN = parseInt(set);
  const repN = parseInt(rep);
  const weightN = parseInt(weight);
  addSessionTrackerEntry(today, name, setN, repN, weightN);
}
