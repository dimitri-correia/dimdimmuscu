import CommonDB from "./CommonDB";
import { ExercisesEntry } from "../logic/ExercisesListLogic";

const exercisesEntriesDBName: string = "exercises_entries";
const exId: string = "id";
const exExerciseName: string = "exercise_name";
const exPrimaryMuscleGroup: string = "primary_muscle";
const exSecondaryMuscleGroup: string = "secondary_muscle";

const muscleGroupEntriesDBName: string = "muscle_group_entries";
const muscleId: string = "id";
const muscleMuscleGroupName: string = "muscle_group_name";

export const createExercisesTableS = () => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `CREATE TABLE IF NOT EXISTS ${exercisesEntriesDBName} (
      ${exId} INTEGER PRIMARY KEY AUTOINCREMENT,
      ${exExerciseName} TEXT NOT NULL UNIQUE,
      ${exPrimaryMuscleGroup} INTEGER NOT NULL,
      ${exSecondaryMuscleGroup} INTEGER,
      FOREIGN KEY (${exPrimaryMuscleGroup}) REFERENCES ${muscleGroupEntriesDBName}(${muscleId}),
      FOREIGN KEY (${exSecondaryMuscleGroup}) REFERENCES ${muscleGroupEntriesDBName}(${muscleId})
  );`
    );
    tx.executeSql(
      `CREATE TABLE IF NOT EXISTS ${muscleGroupEntriesDBName} (
      ${muscleId} INTEGER PRIMARY KEY AUTOINCREMENT,
      ${muscleMuscleGroupName} TEXT NOT NULL UNIQUE);`
    );
  });
};

export const getMuscleGroupEntries = (): Promise<Map<number, string>> => {
  return new Promise<Map<number, string>>((resolve, _) => {
    CommonDB.transaction((tx) => {
      tx.executeSql(
        `SELECT * FROM ${exercisesEntriesDBName};`,
        [],
        (_, result) => {
          const muscleGroupEntries: Map<number, string> = new Map();
          for (let i = 0; i < result.rows.length; i++) {
            const row = result.rows.item(i);
            muscleGroupEntries.set(row.id, row.name);
          }
          resolve(muscleGroupEntries);
        }
      );
    });
  });
};

export const getExerciseEntries = (): Promise<ExercisesEntry[]> => {
  return new Promise<ExercisesEntry[]>((resolve, _) => {
    CommonDB.transaction((tx) => {
      tx.executeSql(
        `SELECT * FROM ${exercisesEntriesDBName};`,
        [],
        (_, result) => {
          const exercisesEntries: ExercisesEntry[] = [];
          for (let i = 0; i < result.rows.length; i++) {
            const row = result.rows.item(i);
            exercisesEntries.push({
              id: row.id,
              name: row.name,
              primaryMuscleGroupId: row.primaryMuscleGroupId,
              secondaryMuscleGroupId: row.secondaryMuscleGroupId,
            });
          }
          resolve(exercisesEntries);
        }
      );
    });
  });
};

export const addMuscleGroupEntry = (name: string) => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `INSERT INTO ${muscleGroupEntriesDBName} ${muscleMuscleGroupName} VALUES ?;`,
      [name]
    );
  });
  console.log(name);
};

export const addExerciseEntry = (
  name: string,
  primaryMuscleGroupId: number,
  secondaryMuscleGroupId: number
) => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `INSERT INTO ${exercisesEntriesDBName} (${exExerciseName},${exPrimaryMuscleGroup},${exSecondaryMuscleGroup}) VALUES (?,?,?);`,
      [name, primaryMuscleGroupId, secondaryMuscleGroupId]
    );
  });
};
