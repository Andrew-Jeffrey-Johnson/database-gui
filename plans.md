I am trying to make a gui using egui in rust. The gui will include all my professional experiences. Each experience has achievements. 
I want to select experiences and achievements I want to include on my resume for a particular job application. 

# postal_address
* id
* name
* line_1
* line_2
* line_3
* city
* state
* zip_code

# contact
* id
* first_name
* last_name
* username
* email
* phone_number
* url
* address_id

# employing_entity
* id
* name
* url
* headquarters_postal_address_id

# experience
* id
* employing_entity_id
* start_timestamptz
* end_timestamptz
* postal_address_id
## achievement
* id
* experience_id
* short_description
* defense
## achievement_variant
* id
* achievement_id
* description
* defense

# project
* id
* start_timestamptz
* end_timestamptz
* name
* url
* postal_address_id
## project_highlight
* id
* project_id
* short_description
* defense
## project_highlight_variant
* id
* project_highlight_id
* description
* defense

# listing_host
* id
* name
* url
## listing
* id
* listing_host_id
* description
* posted_timestamptz
* recruiter_contact_id

# application
* id
* start_timestamptz
* submitted_timestamptz
* listing_id
## application_selection
* id
* application_id
* achievement_variant_id
## application_question_answer
* id
* application_id
* question
* answer
