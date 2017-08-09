use average::Average;
use circular_buffer::CircularBuffer;
use std::fmt::Display;
use json_display::JsonDisplay;

pub struct Historic<T> {
    circular_buffer : CircularBuffer<T>,
    limit : usize 
}

impl<T : JsonDisplay + Display + Average<T>> Historic<T> {
    pub fn reduce(historics : Vec<Historic<T>>) {
        let mut i = 0;
        let mut average_data = None;
        for mut historic in historics {
            // look if the previous historic produce an average data to add to the next historic
            match average_data {
                Some(data) => {historic.circular_buffer.put_item(data);}
                None => {},
            }
            if historic.circular_buffer.get_nb_items() > historic.limit {
                let nb_elements_to_sum = historic.limit / 2;
                println!("Reduction queue {} nbElementsToSum = {}\n", i, nb_elements_to_sum);
                // Accumulate on first nb_elements_to_sum element of the historic
                let mut j = 0;
                let mut accumulator_data = T::empty_cumulator();
                while j < nb_elements_to_sum {
                    match historic.circular_buffer.peek_item(j) {
                            Some(data) => {data.cumulate(&mut accumulator_data);},
                            None => panic!("Problem of implementation in historic reduce method")
                        }
                        j += 1;
                }
                //remove elements used for accumulation from the historic
                j = 0;
                while j < nb_elements_to_sum {
                    historic.circular_buffer.get_item();
                    j += 1;
                }
                // The average_data will be add to the next historic (if historic exists, otherwise it will be lost)
                average_data = Some(T::divide(&accumulator_data, nb_elements_to_sum));
                //{
                    //char json[240];
                    //char *pJson = json;
                    //int remainingSize = sizeof(json);
                    //json[0]= '\0';
                    //printf("Reducted value : %s\n", tData_json(&writeData, &pJson, &remainingSize));
                //}
                i += 1;
            }
            else {
                break;
            }
        }
    }
}

