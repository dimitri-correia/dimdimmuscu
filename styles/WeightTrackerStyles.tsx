import { StyleSheet } from "react-native";

export const pageStyles = StyleSheet.create({
  row: {
    flexDirection: "row",
    justifyContent: "space-between",
    alignItems: "center",
    paddingVertical: 8,
    paddingHorizontal: 16,
    borderBottomWidth: 1,
    borderBottomColor: "#ccc",
    width: "100%",
  },
  column: {
    flex: 1,
    textAlign: "center",
    color: "blue",
    fontSize: 16,
  },
});

export const addWeightStyles = StyleSheet.create({
  addWeightContainer: {
    flexDirection: "row",
    alignItems: "center",
    marginBottom: 16,
  },
  addWeightLabel: {
    fontSize: 16,
    marginRight: 8,
  },
  addWeightInput: {
    flex: 1,
    borderWidth: 1,
    borderColor: "#ccc",
    borderRadius: 4,
    padding: 8,
    marginRight: 8,
    backgroundColor: "#444",
    color: "#fff",
  },
});
