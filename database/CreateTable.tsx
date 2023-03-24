import { createWeightTable } from "./WeightTrackerDB";
import { createExercisesTableS } from "./ExercisesListDB";
import { createPRTable } from "./PersonalRecordDB";
import { createInfosTable } from "./InfosDB";
import { createImageTable } from "./PictureEvolutionDB";
import { createCardioTable } from "./CardioDB";
import { createCaloriesTable } from "./CaloriesDB";

export function createAllTables() {
  createWeightTable();
  createExercisesTableS();
  createPRTable();
  createInfosTable();
  createImageTable();
  createCardioTable();
  createCaloriesTable();
}
