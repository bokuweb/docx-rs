# docx directory
OUTPUT_DIR=../../output/js

# For more information for this image, see  https://github.com/yuntara/docx-png-converter
REPOSITORY_BASE:=yuntara
IMAGE:=${REPOSITORY_BASE}/docx-png-converter

# PNG dpi
DENSITY:=150

pull:
	docker pull ${IMAGE}

run:
	cat ${INPUT} | docker exec -i docx-png-converter bash /root/run.sh ${DENSITY} > ${OUTPUT}

start:
	make stop
	make pull
	docker run --name docx-png-converter -itd ${IMAGE} /bin/bash 

convert-all: 
	make start
	OUTPUT_DIR=${OUTPUT_DIR} node index.mjs
	make stop

stop:
	- docker kill docx-png-converter
	- docker rm docx-png-converter

