typedef struct User { 
	char* name; 
	int age; 
	struct School school; 
	double money; 
	char id; 
	bool isUsed; 
	struct User emergencyContact; 
	int phoneNumber; 
	User* students; 
	User* faculty; 
}; User
 
