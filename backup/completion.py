from random import sample, random
from itertools import product as iter_product

import cvxpy as cp
import numpy as np


def complete_matrix_ext(m: int, n: int, matrix: list, samples: list[tuple[int, int]] = None) -> list:
	assert len(matrix) == m*n

	np_matrix = np.array(matrix).reshape((m, n))
	res = complete_matrix(np_matrix, samples)

	return res.reshape(-1).tolist()

def complete_matrix(matrix: np.ndarray, samples: set[tuple[int, int]] = None) -> np.ndarray:
	m, n = matrix.shape

	if samples is None:
		samples = set()
		for i in range(m):
			for j in range(n):
				if matrix[i, j] != 0:
					samples.add((i, j))

	X = cp.Variable((m, n))

	constraints = [
		X[i, j] == matrix[i, j] for i, j in samples
	]
	objective = cp.Minimize(cp.norm(X, "nuc"))

	prob = cp.Problem(objective, constraints)
	prob.solve(solver=cp.SCS)

	return X.value


if __name__ == "__main__":
	m, n = 100, 200
	rand_matrix = np.zeros((m, n))
	r = 3
	for _ in range(r):
		u = np.random.random((m, 1))
		v = np.random.random((1, n))
		s = random()*100
		rand_matrix += s * u @ v

	k = 7000
	samples = set(sample(list(iter_product(range(m), range(n))), k=k))

	sampled_matrix = np.zeros((m, n))
	for i, j in samples: sampled_matrix[i, j] = rand_matrix[i, j]

	X = complete_matrix(sampled_matrix)

	print("Max error: ", np.max(np.abs(X-rand_matrix)))

	# matrix = np.array([[1, 2, 3, 4, 5],
	# 				   [2, 4, 6, 8, 10],
	# 				   [3, 6, 9, 12, 15]])

	# sampled = np.array([[1, 0, 3, 0, 5],
	# 					[2, 4, 0, 8, 0],
	# 					[0, 0, 9, 12, 15]])
	# X = complete_matrix(sampled)
	
	# print("original:")
	# print(matrix)
	# print("recovered:")
	# print(X)