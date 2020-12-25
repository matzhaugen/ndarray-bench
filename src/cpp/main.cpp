#include <iostream>
#include <Eigen/Core>
#include <Eigen/Dense>
#include <Eigen/Sparse>
using namespace Eigen;
using namespace std;

int main() {
	int p = 1000;
	MatrixXd matrix1 = MatrixXd::Zero(p, p);
	MatrixXd matrix2 = MatrixXd::Ones(p, p);
	MatrixXd matrix3(p, p);
		
	auto start = chrono::steady_clock::now();
	matrix1 = matrix2;
	auto end = chrono::steady_clock::now();
	cout << "Elapsed time in microseconds : "
	  << chrono::duration_cast<chrono::microseconds>(end - start).count()
	  << " us" << endl;

	start = chrono::steady_clock::now();
	matrix3 = matrix2.cwiseProduct(matrix1);
	end = chrono::steady_clock::now();
	cout << "Elapsed time in microseconds : "
	  << chrono::duration_cast<chrono::microseconds>(end - start).count()
	  << " us" << endl;
    
    return 0;
}
