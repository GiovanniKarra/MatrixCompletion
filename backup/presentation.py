import pandas as pd
import numpy as np
from completion import complete_matrix


if __name__ == "__main__":
	students = pd.read_csv("students.csv")
	courses = pd.read_csv("courses.csv")
	data = pd.read_csv("ratings.csv")	

	# Create a pivot table where rows are students, columns are courses, and values are scores
	pivot_table = data.pivot(index='student', columns='course', values='rating')

	# Optionally, you can fill missing scores with a specific value (e.g., NaN or 0)
	pivot_table = pivot_table.dropna(axis=1, how="all")
	pivot_table = pivot_table.fillna(0)  # Replace 0 with `pd.NA` if you prefer NaN

	# Add student names as an index if desired
	pivot_table.index = pivot_table.index.map(students.set_index('id')['name'])

	# Add course names as column headers if desired
	pivot_table.columns = pivot_table.columns.map(courses.set_index('id')['name'])

	# Result
	# print(pivot_table)

	# pivot_table.to_csv("base_matrix.csv")

	arr = pivot_table.to_numpy()
	
	recovered = complete_matrix(arr)
	print(np.linalg.matrix_rank(recovered))
	pivot_table.iloc[:, :] = np.round(recovered)

	# pivot_table.to_csv("output.csv")
	print(pivot_table)