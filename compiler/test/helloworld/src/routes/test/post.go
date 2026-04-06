// Package routes
package routes

type OmniContext struct{}

func POST(hi OmniContext) string {
	return "Hello World"
}
