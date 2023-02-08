TARGET = guess_number

all: $(TARGET)

$(TARGET): src/main.rs
	rustc $<

clean:
	rm -f $(TARGET)
