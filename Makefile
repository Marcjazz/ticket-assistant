# Makefile for running the ticker assistant Dockerfile 

# Target to build the project
build:
	docker build -t ticket_assistant .

exec:
	if [ -z "$GITHUB_TOKEN" ] || [ -z "$GITHUB_REPOSITORY" ] || [ -z "$PR_NUMBER" ]; then
		echo "Missing one the required the following variables: GITHUB_TOKEN, GITHUB_REPOSITORY, PR_NUMBER"
	else 
		docker run -it --rm ticket_assistant $1 $2
	fi


# Combined target to start services and initialize the replica set
start:
	$(MAKE) build # build action
	${MAKE} exec # execute action


.PHONY: up init-replica down start