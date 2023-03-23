import {
  addSessionTrackerLiftEntry,
  addSessionTrackerSetEntry,
  getSessionTrackerLiftEntries,
  getSessionTrackerSetEntries,
} from "../database/SessionTrackerDB";

export interface SessionTrackerLiftEntry {
  id: number;
  date: Date;
  ex: number;
}

export interface SessionTrackerSetEntry {
  id: number;
  set: number;
  rep: number;
  weight: number;
}

export function refreshSessionTrackerLiftEntries(
  setLiftEntries: (
    value:
      | ((prevState: SessionTrackerLiftEntry[]) => SessionTrackerLiftEntry[])
      | SessionTrackerLiftEntry[]
  ) => void
) {
  getSessionTrackerLiftEntries()
    .then((ce: SessionTrackerLiftEntry[]) => {
      setLiftEntries(ce);
    })
    .catch(() => console.debug("error fetching Session Tracker lift entries"));
}

export function refreshSessionTrackerSetEntries(
  id: number,
  setSetEntries: (
    value:
      | ((prevState: SessionTrackerSetEntry[]) => SessionTrackerSetEntry[])
      | SessionTrackerSetEntry[]
  ) => void
) {
  getSessionTrackerSetEntries(id)
    .then((ce: SessionTrackerSetEntry[]) => {
      setSetEntries(ce);
    })
    .catch(() => console.debug("error fetching Session Tracker set entries")); // todo use a map idLift: list(stse)
}

export function addSessionTrackerLift(ex: number) {
  const today: string = new Date().toISOString().split("T")[0];
  addSessionTrackerLiftEntry(today, ex);
}

export function addSessionTrackerSet(
  idLift: number,
  set: string,
  rep: string,
  weight: string
) {
  const setN = parseInt(set);
  const repN = parseInt(rep);
  const weightN = parseInt(weight);
  addSessionTrackerSetEntry(idLift, setN, repN, weightN);
}
