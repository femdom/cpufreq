/*
 *  (C) 2004-2009  Dominik Brodowski <linux@dominikbrodowski.de>
 *
 *  Licensed under the terms of the GNU GPL License version 2.
 */


#include <stdio.h>
#include <errno.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "cpufreq.h"
#include "sysfs.h"

#define BUFFER_SIZE 512
#define MAX_CPUS 2

struct Cpu {
  long freq;
  long latency;
  long limits[2];
  long policy_min;
  long policy_max;
  char policy_governor[BUFFER_SIZE];
};

static struct Cpu all_cpus[MAX_CPUS] = {
  {2400000, 1000, {100000, 1000000}, 100000, 1000000, {"performance"}},
  {-EACCES, -EACCES, {-EACCES, -EACCES}, -EACCES, -EACCES, {"a"}}
};


unsigned long _process_result(long result) {
  if (result >= 0) {
    return result;
  } else {
    errno = -result;
    return result;
  }
}

int cpufreq_cpu_exists(unsigned int cpu) {
  return cpu <= 1 ? 0 : -1;
}

unsigned long cpufreq_get_freq_kernel(unsigned int cpu) {
  if (cpu >= MAX_CPUS) {
    errno = ENOENT;
    return -ENOENT;
  }

  return _process_result(all_cpus[cpu].freq);
}

unsigned long cpufreq_get_freq_hardware(unsigned int cpu)
{
  if (cpu >= MAX_CPUS) {
    errno = ENOENT;
    return -ENOENT;
  }

  if (geteuid() != 0) {
    errno = EACCES;
    return 0;
  }

  return cpufreq_get_freq_kernel(cpu);
}

unsigned long cpufreq_get_transition_latency(unsigned int cpu)
{
  if (cpu >= MAX_CPUS) {
    errno = ENOENT;
    return -ENOENT;
  }

  return _process_result(all_cpus[cpu].latency);
}

int cpufreq_get_hardware_limits(unsigned int cpu,
                                unsigned long *min,
                                unsigned long *max)
{
  if (cpu >= MAX_CPUS) {
    errno = ENOENT;
    return -ENOENT;
  }

  if ((!min) || (!max))
    return -EINVAL;

  long cpu_min = all_cpus[cpu].limits[0];
  long cpu_max = all_cpus[cpu].limits[1];

  if (cpu_min < 0) {
    errno = -cpu_min;
    return cpu_min;
  }

  if (cpu_max < 0) {
    errno = -cpu_max;
    return cpu_max;
  }

  *min = cpu_min;
  *max = cpu_max;

  return 0;
}

char *cpufreq_get_driver(unsigned int cpu)
{
  if (cpu >= MAX_CPUS) {
    errno = ENOENT;
    return NULL;
  }

  return sysfs_get_freq_driver(cpu);
}

void cpufreq_put_driver(char *ptr)
{
  if (!ptr)
    return;
  free(ptr);
}

struct cpufreq_policy *cpufreq_get_policy(unsigned int cpu) {
  if (cpu >= MAX_CPUS) {
    errno = ENOENT;
    return NULL;
  }

  long min = all_cpus[cpu].policy_min;
  long max = all_cpus[cpu].policy_max;

  if (min < 0) {
    errno = -max;
    return NULL;
  }

  if (max < 0) {
    errno = -max;
    return NULL;
  }

  struct cpufreq_policy* policy = malloc(sizeof(struct cpufreq_policy));

  if (!policy) {
    return NULL;
  }

  policy->governor = strndup(all_cpus[cpu].policy_governor, BUFFER_SIZE);
  if (!policy->governor) {
    free(policy);
    return NULL;
  }

  policy->min = min;
  policy->max = max;

  return policy;
}

void cpufreq_put_policy(struct cpufreq_policy *policy)
{
  if ((!policy) || (!policy->governor))
    return;

  free(policy->governor);
  policy->governor = NULL;
  free(policy);
}

struct cpufreq_available_governors *cpufreq_get_available_governors(unsigned int cpu)
{
  if (cpu >= MAX_CPUS) {
    errno = ENOENT;
    return NULL;
  }

  struct cpufreq_available_governors *first = NULL;
  struct cpufreq_available_governors *current = NULL;

  char* governors[] = {
    "conservative", "ondemand", "powersave", "userspace", "performance"
  };

  unsigned long i;
  for(i = 0; i < 5; i++) {
    if (governors[i] == NULL) {
      continue;
    }

    struct cpufreq_available_governors *next = malloc(sizeof(struct cpufreq_available_governors));

    if (!next) {
      cpufreq_put_available_governors(first);
      return NULL;
    }

    if (current) {
      current->next = next;
    }
    current = next;
    current->governor = strndup(governors[i], BUFFER_SIZE);

    if (!current->governor) {
      cpufreq_put_available_governors(first);
      return NULL;
    }

    if (first == NULL) {
      first = current;
    }

    current->first = first;
  }

  return first;
}

void cpufreq_put_available_governors(struct cpufreq_available_governors *any)
{
  struct cpufreq_available_governors *tmp, *next;

  if (!any)
    return;

  tmp = any->first;
  while (tmp) {
    next = tmp->next;
    if (tmp->governor)
      free(tmp->governor);
    free(tmp);
    tmp = next;
  }
}


struct cpufreq_available_frequencies
*cpufreq_get_available_frequencies(unsigned int cpu)
{
  if (cpu >= MAX_CPUS) {
    errno = ENOENT;
    return NULL;
  }

  return sysfs_get_available_frequencies(cpu);
}

void cpufreq_put_available_frequencies(struct cpufreq_available_frequencies *any) {
  struct cpufreq_available_frequencies *tmp, *next;

  if (!any)
    return;

  tmp = any->first;
  while (tmp) {
    next = tmp->next;
    free(tmp);
    tmp = next;
  }
}


struct cpufreq_affected_cpus *cpufreq_get_affected_cpus(unsigned int cpu) {
  if (cpu >= MAX_CPUS) {
    errno = ENOENT;
    return NULL;
  }

  return sysfs_get_freq_affected_cpus(cpu);
}

void cpufreq_put_affected_cpus(struct cpufreq_affected_cpus *any)
{
  struct cpufreq_affected_cpus *tmp, *next;

  if (!any)
    return;

  tmp = any->first;
  while (tmp) {
    next = tmp->next;
    free(tmp);
    tmp = next;
  }
}


struct cpufreq_affected_cpus *cpufreq_get_related_cpus(unsigned int cpu)
{
  if (cpu >= MAX_CPUS) {
    errno = ENOENT;
    return NULL;
  }

  return sysfs_get_freq_related_cpus(cpu);
}

void cpufreq_put_related_cpus(struct cpufreq_affected_cpus *any)
{
  cpufreq_put_affected_cpus(any);
}


int cpufreq_set_policy(unsigned int cpu, struct cpufreq_policy *policy)
{
  if (!policy || !(policy->governor))
    return -EINVAL;

  if (cpu >= MAX_CPUS) {
    errno = ENOENT;
    return -ENOENT;
  }

  if (geteuid() != 0) {
    errno = EACCES;
    return -EACCES;
  }

  int min = all_cpus[cpu].policy_min;
  int max = all_cpus[cpu].policy_max;

  if (min < 0) {
    errno = -min;
    return min;
  }

  if (max < 0) {
    errno = -max;
    return max;
  }

  all_cpus[cpu].policy_min = policy->min;
  all_cpus[cpu].policy_max = policy->max;
  strncpy(all_cpus[cpu].policy_governor, policy->governor, BUFFER_SIZE);

  return 0;
}


int cpufreq_modify_policy_min(unsigned int cpu, unsigned long min_freq)
{
  if (MAX_CPUS <= cpu) {
    errno = ENOENT;
    return -ENOENT;
  }

  if (geteuid() != 0) {
    errno = EACCES;
    return -EACCES;
  }

  all_cpus[cpu].policy_min = min_freq;
  return 0;
}


int cpufreq_modify_policy_max(unsigned int cpu, unsigned long max_freq)
{
  if (MAX_CPUS <= cpu) {
    errno = ENOENT;
    return -ENOENT;
  }

  if (geteuid() != 0) {
    errno = EACCES;
    return -EACCES;
  }

  all_cpus[cpu].policy_max = max_freq;
  return 0;
}

int cpufreq_modify_policy_governor(unsigned int cpu, char *governor)
{
  if (MAX_CPUS <= cpu) {
    errno = ENOENT;
    return -ENOENT;
  }

  if (geteuid() != 0) {
    errno = EACCES;
    return -EACCES;
  }

  strncpy(all_cpus[cpu].policy_governor, governor, BUFFER_SIZE);
  return 0;
}

int cpufreq_set_frequency(unsigned int cpu, unsigned long target_frequency)
{
  if (cpu >= MAX_CPUS) {
    errno = ENOENT;
    return -ENOENT;
  }

  if (geteuid() != 0) {
    errno = EACCES;
    return -EACCES;
  }

  int result = all_cpus[cpu].freq;

  if (result < 0) {
    errno = -result;
    return result;
  }

  all_cpus[cpu].freq = target_frequency;

  return 0;
}

struct cpufreq_stats *cpufreq_get_stats(unsigned int cpu,
                                        unsigned long long *total_time)
{
  if (cpu >= MAX_CPUS) {
    errno = ENOENT;
    return NULL;
  }

  return sysfs_get_freq_stats(cpu, total_time);
}

void cpufreq_put_stats(struct cpufreq_stats *any)
{
  struct cpufreq_stats *tmp, *next;

  if (!any)
    return;

  tmp = any->first;
  while (tmp) {
    next = tmp->next;
    free(tmp);
    tmp = next;
  }
}

unsigned long cpufreq_get_transitions(unsigned int cpu)
{
  if (cpu >= MAX_CPUS) {
    errno = ENOENT;
    return 0;
  }

  return sysfs_get_freq_transitions(cpu);
}
