// Package routes
package routes

type OmniContext struct{}

func MIDDLEWARE(hi OmniContext) string {
	return "Hello World"
}
