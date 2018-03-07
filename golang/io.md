- Argument parsing
  - os.Args : ``` var Args []string ```
  - flag.Parse(), flag.XXXXVar(...)

- Console
  - fmt.Printf(...)
  - fmt.Scanf(...)

- Text file
  - read
    ```
    package main

    import ( 
      "bufio"
      "fmt"
      "os"
    )

    func main() {
      file, err := os.Open("text.txt")
      if err != nil {
        fmt.Printf("file open error!\n")
        return
      }
      defer file.Close()
      
      scanner := bufio.NewScanner(file)
      for scanner.Scan() {
        fmt.Println(scanner.Text())
      }
      if err := scanner.Err(); err != nil {
        fmt.Fprintln(os.Stderr, "reading standard input:", err)
      }
    }
    ```
  - ...
  
- Binary file
  - read
    ```
    package main

    import (
      "bufio"
      "encoding/binary"
      "fmt"
      "os"
    )

    func main() {
      file, err := os.Open("text.dat")
      if err != nil {
        fmt.Printf("file open error!\n")
        return
      }
      defer file.Close()
      
      reader := bufio.NewReader(file)
      
      var x32 int32
      if err := binary.Read(reader, binary.LittleEndian, &x32); err != nil {
        fmt.Println("ERROR! (maybe EOF)")
      }
      
      buff := make([]byte, buffSize)
      if n, err := reader.Read(buff), n == 0 && err == io.EOF {
        fmt.Println("EOF")
      }
    }
    ```
    
  - write
    ```
    package main

    import (
      "bufio"
      "encoding/binary"
      "fmt"
      "os"
    )

    func main() {
      file, err := os.Create("text.dat")
      if err != nil {
        fmt.Printf("file open error!\n")
        return
      }
      defer file.Close()
      
      writer := bufio.NewWriter(file)
      
      //... data := make([]byte, size)
      
      if n, err := writer.Write(data); err != nil {
        fmt.Printf("ERROR: %v\n", err)
      }
    }
    ```
    
