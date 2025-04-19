# Create a JSON array with a single time entry
# Using compact JSON to reduce potential formatting errors
time_entries="[{\"id\":\"hello-1\",\"description\":\"Hello from bash plugin\",\"project_id\":\"proj-1\",\"project_name\":\"Example Project\",\"customer_id\":\"cust-1\",\"customer_name\":\"Example Customer\",\"started_at\":\"$now\",\"ended_at\":\"$later\",\"tags\":[\"example\",\"bash\"],\"source\":\"hello\",\"source_url\":null,\"billable\":true}]" 