import { averageXdays, WeightEntry } from "../logic/WeightTrackerLogic";

function getDate(numberOfDays: number) {
  return new Date(new Date().setDate(new Date().getDate() + numberOfDays));
}

const entriesAllPresent: WeightEntry[] = [
  // id AND dates have no gap
  { id: 1, weight: 80, date: getDate(0) },
  { id: 2, weight: 81, date: getDate(1) },
  { id: 3, weight: 82, date: getDate(2) },
  { id: 4, weight: 83, date: getDate(3) },
  { id: 5, weight: 84, date: getDate(4) },
];
const entriesNotAllPresent: WeightEntry[] = [
  // id have no gap but dates does
  // day 3 not created
  { id: 1, weight: 80, date: getDate(0) },
  { id: 2, weight: 81, date: getDate(1) },
  { id: 3, weight: 82, date: getDate(3) },
  { id: 4, weight: 83, date: getDate(4) },
];
const entriesWithSomeDeletion: WeightEntry[] = [
  // id have no gap but dates does
  // day 2 deleted
  { id: 1, weight: 80, date: getDate(0) },
  { id: 3, weight: 81, date: getDate(2) },
  { id: 4, weight: 82, date: getDate(3) },
  { id: 4, weight: 83, date: getDate(4) },
];

describe("test averageXdays function", () => {
  const date: Date = getDate(4);
  it("should calculates the average of weights of the last 2 days correctly when there is more data than that", () => {
    const numberOfDays = 2;
    const result = averageXdays(entriesAllPresent, date, numberOfDays);
    expect(result).toBe(83.5); // 84,83
  });
  it("should calculates the average of weights of the last 8 days correctly when only five are presents", () => {
    const numberOfDays = 8;
    const result = averageXdays(entriesAllPresent, date, numberOfDays);
    expect(result).toBe(82); // 84,83,82,81,80
  });
  it("calculates the average of weights correctly when a Date is missing", () => {
    const numberOfDays = 4;
    const result = averageXdays(entriesNotAllPresent, date, numberOfDays);
    expect(result).toBe(82); // 83,82,81
  });
  it("should calculates the average of weights correctly when a Date have been deleted", () => {
    const numberOfDays = 3;
    const result = averageXdays(entriesWithSomeDeletion, date, numberOfDays);
    expect(result).toBe(82); // 83,82,81
  });
});
