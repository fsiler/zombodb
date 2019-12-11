/**
 * Copyright 2019 ZomboDB, LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#include "postgres.h"

#ifndef ZOMBODB_HELPER_FUNCS_H
#define ZOMBODB_HELPER_FUNCS_H

PGDLLEXPORT void zdb_log_proxy(int loglevel, const char *log_msg);

#endif //ZOMBODB_HELPER_FUNCS_H