import {
  addExerciseEntry,
  addMuscleGroupEntry,
  getExerciseEntries,
  getMuscleGroupEntries,
} from "../database/ExercisesListDB";
import * as TextEL from "../assets/texts/ExercisesListTexts";
import { confirmationChanges } from "../components/commons/ValidateChanges";

export interface ExercisesEntry {
  id: number;
  name: string;
  primaryMuscleGroupId: number;
  secondaryMuscleGroupId: number;
}

export function refreshExercisesAndMuscleEntries(
  setExerciseEntries: (
    value:
      | ((prevState: ExercisesEntry[]) => ExercisesEntry[])
      | ExercisesEntry[]
  ) => void,
  setMuscleGroupEntries: (
    value:
      | ((prevState: Map<number, string>) => Map<number, string>)
      | Map<number, string>
  ) => void
) {
  getExerciseEntries()
    .then((ex: ExercisesEntry[]) => {
      setExerciseEntries(ex);
    })
    .catch(() => console.debug("error fetching exercise entries"));
  getMuscleGroupEntries()
    .then((mg: Map<number, string>) => {
      setMuscleGroupEntries(mg);
    })
    .catch(() => console.debug("error fetching muscle group entries"));
}

export function addExercise(
  setModalEx: (value: ((prevState: boolean) => boolean) | boolean) => void,
  modalExName: string,
  modalExPrimary: number,
  modalExSecondary: number,
  exerciseEntry: ExercisesEntry[]
) {
  if (
    !modalExName ||
    exerciseEntry.map((ex) => ex.name).includes(modalExName)
  ) {
    alert(TextEL.incorrectName);
    return;
  }
  if (modalExPrimary == modalExSecondary) {
    alert(TextEL.primarySecondaryCannotBeTheSame);
    return;
  }
  confirmationChanges(() => {
    addExerciseEntry(modalExName, modalExPrimary, modalExSecondary);
    setModalEx(false);
  });
}

export function addMuscleGroup(
  setModalGM: (value: ((prevState: boolean) => boolean) | boolean) => void,
  modalGMName: string,
  muscleGroupEntries: Map<number, string>
) {
  if (
    !modalGMName ||
    Array.from(muscleGroupEntries.values()).includes(modalGMName)
  ) {
    alert(TextEL.incorrectName);
    return;
  }
  confirmationChanges(() => {
    addMuscleGroupEntry(modalGMName);
    setModalGM(false);
  });
}
