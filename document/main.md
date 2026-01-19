# Design Document

**Author:** E.A.S.

## Purpose
I have trouble organizing my ideas

## Idea

### Concepts
- Project: Something that has to be done before a certain date.
- ProjectNode: A Project stored within a specific database (either internal or external), identifiable by a unique path. It includes the Project's metadata, associated tasks, and references to subprojects or databases.
- Task: Something that needs to be done so a Project can be completed.
- Root: The main (manager) database that contains Projects, Tasks, and metadata.
- External Database: A separate database that may contain Projects not stored in the Root.

### Projects
Any Project may have other subprojects as part of it and may also have Tasks. Each ProjectNode may have a due date and a priority. The estimated time a ProjectNode may take is calculated by adding the time all associated Tasks and subprojects will take. The difficulty score may alter the estimated time. The difficulty score of a ProjectNode is calculated by summing the difficulty scores of all Tasks and subprojects.

Each ProjectNode may be stored in its own external database or within the manager (Root) database. External databases may be located using a URL or a file path.

### Task
Any Project may have Tasks associated with it. Each Task may have a difficulty score. All Tasks must be stored in the same database as the ProjectNode they belong to.

### Extras
Projects and Tasks may have descriptions and tags.

## Architecture

### Project Storage and Identification
A ProjectNode stored in the manager database has a simple numeric ID. If the ProjectNode is stored in an external database, its identification is a two-part path: the ID of the external storage and the ID of the ProjectNode within that storage. If the ProjectNode itself references another external database, the ID path extends with each layer of redirection.

**Example:**
- A ProjectNode with ID 3 stored at external storage 2 will have the path: `[2, 3]`.
- If that ProjectNode has a subproject with ID 7 stored at external storage 5, the path relative to the parent would be: `[5, 7]`, and the absolute path from Root would be: `[2, 5, 7]`.
- If multiple ProjectNodes reference the same subproject or the Root holds a pointer to that external database, the subproject may have multiple valid absolute paths.

Effectively, this structure creates a recursive pathing system similar to file system paths, enabling both relative and absolute navigation of ProjectNodes across databases.

## Interaction
