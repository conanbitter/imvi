package main

import (
	"github.com/veandco/go-sdl2/img"
	"github.com/veandco/go-sdl2/sdl"
)

type WorkerTask struct {
	filename     string
	index        int
	is_thumbnail bool
}

type WorkerResult struct {
	is_thumbnail bool
	surface      *sdl.Surface
	index        int
	err          error
}

const MAX_TASKS = 10

var taskChan chan WorkerTask = make(chan WorkerTask, MAX_TASKS)
var resultChan chan WorkerResult = make(chan WorkerResult, MAX_TASKS)

func LoadingWorker(tasks <-chan WorkerTask, results chan<- WorkerResult) {
	for task := range tasks {
		surface, err := img.Load(task.filename)
		results <- WorkerResult{
			is_thumbnail: task.is_thumbnail,
			index:        task.index,
			surface:      surface,
			err:          err,
		}
	}
}
